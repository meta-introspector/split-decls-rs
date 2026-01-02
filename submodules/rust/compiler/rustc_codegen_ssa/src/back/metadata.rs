mkuse!{use std :: borrow :: Cow ;}
mkuse!{use std :: fs :: File ;}
mkuse!{use std :: io :: Write ;}
mkuse!{use std :: path :: Path ;}
mkuse!{use itertools :: Itertools ;}
mkuse!{use object :: write :: { self , StandardSegment , Symbol , SymbolSection } ;}
mkuse!{use object :: { Architecture , BinaryFormat , Endianness , FileFlags , Object , ObjectSection , ObjectSymbol , SectionFlags , SectionKind , SymbolFlags , SymbolKind , SymbolScope , elf , pe , xcoff , } ;}
mkuse!{use rustc_abi :: Endian ;}
mkuse!{use rustc_data_structures :: memmap :: Mmap ;}
mkuse!{use rustc_data_structures :: owned_slice :: { OwnedSlice , try_slice_owned } ;}
mkuse!{use rustc_metadata :: EncodedMetadata ;}
mkuse!{use rustc_metadata :: creader :: MetadataLoader ;}
mkuse!{use rustc_metadata :: fs :: METADATA_FILENAME ;}
mkuse!{use rustc_middle :: bug ;}
mkuse!{use rustc_session :: Session ;}
mkuse!{use rustc_span :: sym ;}
mkuse!{use rustc_target :: spec :: { RelocModel , Target , ef_avr_arch } ;}
mkuse!{use tracing :: debug ;}
mkuse!{use super :: apple ;}
mkitem!{mkstruct!{# [doc = " The default metadata loader. This is used by cg_llvm and cg_clif."] # [doc = ""] # [doc = " # Metadata location"] # [doc = ""] # [doc = " <dl>"] # [doc = " <dt>rlib</dt>"] # [doc = " <dd>The metadata can be found in the `lib.rmeta` file inside of the ar archive.</dd>"] # [doc = " <dt>dylib</dt>"] # [doc = " <dd>The metadata can be found in the `.rustc` section of the shared library.</dd>"] # [doc = " </dl>"] # [derive (Debug)] pub (crate) struct DefaultMetadataLoader ;}}
mkitem!{static AIX_METADATA_SYMBOL_NAME : & 'static str = "__aix_rust_metadata" ;}

macro_rules! load_metadata_with_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function load_metadata_with in module {}", module_path!());
    };
}

mkfn!{
    load_metadata_with_introspect!();
    fn load_metadata_with (path : & Path , f : impl for < 'a > FnOnce (& 'a [u8]) -> Result < & 'a [u8] , String > ,) -> Result < OwnedSlice , String > { let file = File :: open (path) . map_err (| e | format ! ("failed to open file '{}': {}" , path . display () , e)) ? ; unsafe { Mmap :: map (file) } . map_err (| e | format ! ("failed to mmap file '{}': {}" , path . display () , e)) . and_then (| mmap | try_slice_owned (mmap , | mmap | f (mmap))) }
}
mkitem!{mkimpl!{impl MetadataLoader for DefaultMetadataLoader { fn get_rlib_metadata (& self , target : & Target , path : & Path) -> Result < OwnedSlice , String > { debug ! ("getting rlib metadata for {}" , path . display ()) ; load_metadata_with (path , | data | { let archive = object :: read :: archive :: ArchiveFile :: parse (& * data) . map_err (| e | format ! ("failed to parse rlib '{}': {}" , path . display () , e)) ? ; for entry_result in archive . members () { let entry = entry_result . map_err (| e | format ! ("failed to parse rlib '{}': {}" , path . display () , e)) ? ; if entry . name () == METADATA_FILENAME . as_bytes () { let data = entry . data (data) . map_err (| e | format ! ("failed to parse rlib '{}': {}" , path . display () , e)) ? ; if target . is_like_aix { return get_metadata_xcoff (path , data) ; } else { return search_for_section (path , data , ".rmeta") ; } } } Err (format ! ("metadata not found in rlib '{}'" , path . display ())) }) } fn get_dylib_metadata (& self , target : & Target , path : & Path) -> Result < OwnedSlice , String > { debug ! ("getting dylib metadata for {}" , path . display ()) ; if target . is_like_aix { load_metadata_with (path , | data | { let archive = object :: read :: archive :: ArchiveFile :: parse (& * data) . map_err (| e | { format ! ("failed to parse aix dylib '{}': {}" , path . display () , e) }) ? ; match archive . members () . exactly_one () { Ok (lib) => { let lib = lib . map_err (| e | { format ! ("failed to parse aix dylib '{}': {}" , path . display () , e) }) ? ; let data = lib . data (data) . map_err (| e | { format ! ("failed to parse aix dylib '{}': {}" , path . display () , e) }) ? ; get_metadata_xcoff (path , data) } Err (e) => Err (format ! ("failed to parse aix dylib '{}': {}" , path . display () , e)) , } }) } else { load_metadata_with (path , | data | search_for_section (path , data , ".rustc")) } } }}}

macro_rules! search_for_section_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function search_for_section in module {}", module_path!());
    };
}

mkfn!{
    search_for_section_introspect!();
    pub (super) fn search_for_section < 'a > (path : & Path , bytes : & 'a [u8] , section : & str ,) -> Result < & 'a [u8] , String > { let Ok (file) = object :: File :: parse (bytes) else { return Ok (bytes) ; } ; file . section_by_name (section) . ok_or_else (| | format ! ("no `{}` section in '{}'" , section , path . display ())) ? . data () . map_err (| e | format ! ("failed to read {} section in '{}': {}" , section , path . display () , e)) }
}

macro_rules! add_gnu_property_note_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function add_gnu_property_note in module {}", module_path!());
    };
}

mkfn!{
    add_gnu_property_note_introspect!();
    fn add_gnu_property_note (file : & mut write :: Object < 'static > , architecture : Architecture , binary_format : BinaryFormat , endianness : Endianness ,) { if binary_format != BinaryFormat :: Elf || ! matches ! (architecture , Architecture :: X86_64 | Architecture :: Aarch64) { return ; } let section = file . add_section (file . segment_name (StandardSegment :: Data) . to_vec () , b".note.gnu.property" . to_vec () , SectionKind :: Note ,) ; let mut data : Vec < u8 > = Vec :: new () ; let n_namsz : u32 = 4 ; let n_descsz : u32 = 16 ; let n_type : u32 = object :: elf :: NT_GNU_PROPERTY_TYPE_0 ; let header_values = [n_namsz , n_descsz , n_type] ; header_values . iter () . for_each (| v | { data . extend_from_slice (& match endianness { Endianness :: Little => v . to_le_bytes () , Endianness :: Big => v . to_be_bytes () , }) }) ; data . extend_from_slice (b"GNU\0") ; let pr_type : u32 = match architecture { Architecture :: X86_64 => object :: elf :: GNU_PROPERTY_X86_FEATURE_1_AND , Architecture :: Aarch64 => object :: elf :: GNU_PROPERTY_AARCH64_FEATURE_1_AND , _ => unreachable ! () , } ; let pr_datasz : u32 = 4 ; let pr_data : u32 = 3 ; let pr_padding : u32 = 0 ; let property_values = [pr_type , pr_datasz , pr_data , pr_padding] ; property_values . iter () . for_each (| v | { data . extend_from_slice (& match endianness { Endianness :: Little => v . to_le_bytes () , Endianness :: Big => v . to_be_bytes () , }) }) ; file . append_section_data (section , & data , 8) ; }
}

macro_rules! get_metadata_xcoff_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function get_metadata_xcoff in module {}", module_path!());
    };
}

mkfn!{
    get_metadata_xcoff_introspect!();
    pub (super) fn get_metadata_xcoff < 'a > (path : & Path , data : & 'a [u8]) -> Result < & 'a [u8] , String > { let Ok (file) = object :: File :: parse (data) else { return Ok (data) ; } ; let info_data = search_for_section (path , data , ".info") ? ; if let Some (metadata_symbol) = file . symbols () . find (| sym | sym . name () == Ok (AIX_METADATA_SYMBOL_NAME)) { let offset = metadata_symbol . address () as usize ; if offset < 4 { return Err (format ! ("Invalid metadata symbol offset: {offset}")) ; } let len = u32 :: from_be_bytes (info_data [(offset - 4) .. offset] . try_into () . unwrap ()) as usize ; if offset + len > (info_data . len () as usize) { return Err (format ! ("Metadata at offset {offset} with size {len} is beyond .info section")) ; } Ok (& info_data [offset .. (offset + len)]) } else { Err (format ! ("Unable to find symbol {AIX_METADATA_SYMBOL_NAME}")) } }
}

macro_rules! create_object_file_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function create_object_file in module {}", module_path!());
    };
}

mkfn!{
    create_object_file_introspect!();
    pub (crate) fn create_object_file (sess : & Session) -> Option < write :: Object < 'static > > { let endianness = match sess . target . options . endian { Endian :: Little => Endianness :: Little , Endian :: Big => Endianness :: Big , } ; let Some ((architecture , sub_architecture)) = sess . target . object_architecture (& sess . unstable_target_features) else { return None ; } ; let binary_format = sess . target . binary_format . to_object () ; let mut file = write :: Object :: new (binary_format , architecture , endianness) ; file . set_sub_architecture (sub_architecture) ; if sess . target . is_like_darwin { if macho_is_arm64e (& sess . target) { file . set_macho_cpu_subtype (object :: macho :: CPU_SUBTYPE_ARM64E) ; } file . set_macho_build_version (macho_object_build_version_for_target (sess)) } if binary_format == BinaryFormat :: Coff { let original_mangling = file . mangling () ; file . set_mangling (object :: write :: Mangling :: None) ; let mut feature = 0 ; if file . architecture () == object :: Architecture :: I386 { feature |= 1 ; } file . add_symbol (object :: write :: Symbol { name : "@feat.00" . into () , value : feature , size : 0 , kind : object :: SymbolKind :: Data , scope : object :: SymbolScope :: Compilation , weak : false , section : object :: write :: SymbolSection :: Absolute , flags : object :: SymbolFlags :: None , }) ; file . set_mangling (original_mangling) ; } let e_flags = elf_e_flags (architecture , sess) ; let os_abi = elf_os_abi (sess) ; let abi_version = 0 ; add_gnu_property_note (& mut file , architecture , binary_format , endianness) ; file . flags = FileFlags :: Elf { os_abi , abi_version , e_flags } ; Some (file) }
}

macro_rules! elf_os_abi_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function elf_os_abi in module {}", module_path!());
    };
}

mkfn!{
    elf_os_abi_introspect!();
    pub (super) fn elf_os_abi (sess : & Session) -> u8 { match sess . target . options . os . as_ref () { "hermit" => elf :: ELFOSABI_STANDALONE , "freebsd" => elf :: ELFOSABI_FREEBSD , "solaris" => elf :: ELFOSABI_SOLARIS , _ => elf :: ELFOSABI_NONE , } }
}

macro_rules! elf_e_flags_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function elf_e_flags in module {}", module_path!());
    };
}

mkfn!{
    elf_e_flags_introspect!();
    pub (super) fn elf_e_flags (architecture : Architecture , sess : & Session) -> u32 { match architecture { Architecture :: Mips | Architecture :: Mips64 | Architecture :: Mips64_N32 => { let is_32bit = architecture == Architecture :: Mips ; let mut e_flags = match sess . target . options . cpu . as_ref () { "mips1" if is_32bit => elf :: EF_MIPS_ARCH_1 , "mips2" if is_32bit => elf :: EF_MIPS_ARCH_2 , "mips3" => elf :: EF_MIPS_ARCH_3 , "mips4" => elf :: EF_MIPS_ARCH_4 , "mips5" => elf :: EF_MIPS_ARCH_5 , "mips32r2" if is_32bit => elf :: EF_MIPS_ARCH_32R2 , "mips32r6" if is_32bit => elf :: EF_MIPS_ARCH_32R6 , "mips64r2" if ! is_32bit => elf :: EF_MIPS_ARCH_64R2 , "mips64r6" if ! is_32bit => elf :: EF_MIPS_ARCH_64R6 , s if s . starts_with ("mips32") && ! is_32bit => { sess . dcx () . fatal (format ! ("invalid CPU `{}` for 64-bit MIPS target" , s)) } s if s . starts_with ("mips64") && is_32bit => { sess . dcx () . fatal (format ! ("invalid CPU `{}` for 32-bit MIPS target" , s)) } _ if is_32bit => elf :: EF_MIPS_ARCH_32R2 , _ => elf :: EF_MIPS_ARCH_64R2 , } ; match sess . target . options . llvm_abiname . as_ref () { "o32" if is_32bit => e_flags |= elf :: EF_MIPS_ABI_O32 , "n32" if ! is_32bit => e_flags |= elf :: EF_MIPS_ABI2 , "n64" if ! is_32bit => { } "" if is_32bit => e_flags |= elf :: EF_MIPS_ABI_O32 , "" => sess . dcx () . fatal ("LLVM ABI must be specified for 64-bit MIPS targets") , s if is_32bit => { sess . dcx () . fatal (format ! ("invalid LLVM ABI `{}` for 32-bit MIPS target" , s)) } s => sess . dcx () . fatal (format ! ("invalid LLVM ABI `{}` for 64-bit MIPS target" , s)) , } ; if sess . target . options . relocation_model != RelocModel :: Static { e_flags |= elf :: EF_MIPS_PIC | elf :: EF_MIPS_CPIC ; } if sess . target . options . cpu . contains ("r6") { e_flags |= elf :: EF_MIPS_NAN2008 ; } e_flags } Architecture :: Riscv32 | Architecture :: Riscv64 => { let mut e_flags : u32 = 0x0 ; if sess . unstable_target_features . contains (& sym :: zca) { e_flags |= elf :: EF_RISCV_RVC ; } if sess . unstable_target_features . contains (& sym :: ztso) { e_flags |= elf :: EF_RISCV_TSO ; } match & * sess . target . llvm_abiname { "ilp32" | "lp64" => () , "ilp32f" | "lp64f" => e_flags |= elf :: EF_RISCV_FLOAT_ABI_SINGLE , "ilp32d" | "lp64d" => e_flags |= elf :: EF_RISCV_FLOAT_ABI_DOUBLE , "ilp32e" | "lp64e" => e_flags |= elf :: EF_RISCV_RVE , _ => bug ! ("unknown RISC-V ABI name") , } e_flags } Architecture :: LoongArch32 | Architecture :: LoongArch64 => { let mut e_flags : u32 = elf :: EF_LARCH_OBJABI_V1 ; match & * sess . target . llvm_abiname { "ilp32s" | "lp64s" => e_flags |= elf :: EF_LARCH_ABI_SOFT_FLOAT , "ilp32f" | "lp64f" => e_flags |= elf :: EF_LARCH_ABI_SINGLE_FLOAT , "ilp32d" | "lp64d" => e_flags |= elf :: EF_LARCH_ABI_DOUBLE_FLOAT , _ => bug ! ("unknown LoongArch ABI name") , } e_flags } Architecture :: Avr => { if let Some (ref cpu) = sess . opts . cg . target_cpu { ef_avr_arch (cpu) } else { bug ! ("AVR CPU not explicitly specified") } } Architecture :: Csky => { let e_flags = match sess . target . options . abi . as_ref () { "abiv2" => elf :: EF_CSKY_ABIV2 , _ => elf :: EF_CSKY_ABIV1 , } ; e_flags } Architecture :: PowerPc64 => { const EF_PPC64_ABI_UNKNOWN : u32 = 0 ; const EF_PPC64_ABI_ELF_V1 : u32 = 1 ; const EF_PPC64_ABI_ELF_V2 : u32 = 2 ; match sess . target . options . llvm_abiname . as_ref () { "elfv1" => EF_PPC64_ABI_ELF_V1 , "elfv2" => EF_PPC64_ABI_ELF_V2 , "" if sess . target . options . binary_format . to_object () == BinaryFormat :: Elf => { bug ! ("No ABI specified for this PPC64 ELF target") ; } _ => EF_PPC64_ABI_UNKNOWN , } } _ => 0 , } }
}

macro_rules! macho_object_build_version_for_target_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function macho_object_build_version_for_target in module {}", module_path!());
    };
}

mkfn!{
    macho_object_build_version_for_target_introspect!();
    # [doc = " Mach-O files contain information about:"] # [doc = " - The platform/OS they were built for (macOS/watchOS/Mac Catalyst/iOS simulator etc)."] # [doc = " - The minimum OS version / deployment target."] # [doc = " - The version of the SDK they were targetting."] # [doc = ""] # [doc = " In the past, this was accomplished using the LC_VERSION_MIN_MACOSX, LC_VERSION_MIN_IPHONEOS,"] # [doc = " LC_VERSION_MIN_TVOS or LC_VERSION_MIN_WATCHOS load commands, which each contain information"] # [doc = " about the deployment target and SDK version, and implicitly, by their presence, which OS they"] # [doc = " target. Simulator targets were determined if the architecture was x86_64, but there was e.g. a"] # [doc = " LC_VERSION_MIN_IPHONEOS present."] # [doc = ""] # [doc = " This is of course brittle and limited, so modern tooling emit the LC_BUILD_VERSION load"] # [doc = " command (which contains all three pieces of information in one) when the deployment target is"] # [doc = " high enough, or the target is something that wouldn't be encodable with the old load commands"] # [doc = " (such as Mac Catalyst, or Aarch64 iOS simulator)."] # [doc = ""] # [doc = " Since Xcode 15, Apple's LD apparently requires object files to use this load command, so this"] # [doc = " returns the `MachOBuildVersion` for the target to do so."] fn macho_object_build_version_for_target (sess : & Session) -> object :: write :: MachOBuildVersion { # [doc = " The `object` crate demands \"X.Y.Z encoded in nibbles as xxxx.yy.zz\""] # [doc = " e.g. minOS 14.0 = 0x000E0000, or SDK 16.2 = 0x00100200"] fn pack_version (apple :: OSVersion { major , minor , patch } : apple :: OSVersion) -> u32 { let (major , minor , patch) = (major as u32 , minor as u32 , patch as u32) ; (major << 16) | (minor << 8) | patch } let platform = apple :: macho_platform (& sess . target) ; let min_os = sess . apple_deployment_target () ; let mut build_version = object :: write :: MachOBuildVersion :: default () ; build_version . platform = platform ; build_version . minos = pack_version (min_os) ; build_version . sdk = 0 ; build_version }
}

macro_rules! macho_is_arm64e_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function macho_is_arm64e in module {}", module_path!());
    };
}

mkfn!{
    macho_is_arm64e_introspect!();
    # [doc = " Is Apple's CPU subtype `arm64e`s"] fn macho_is_arm64e (target : & Target) -> bool { target . llvm_target . starts_with ("arm64e") }
}
mkitem!{mkenum!{pub (crate) enum MetadataPosition { First , Last , }}}

macro_rules! create_wrapper_file_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function create_wrapper_file in module {}", module_path!());
    };
}

mkfn!{
    create_wrapper_file_introspect!();
    # [doc = " For rlibs we \"pack\" rustc metadata into a dummy object file."] # [doc = ""] # [doc = " Historically it was needed because rustc linked rlibs as whole-archive in some cases."] # [doc = " In that case linkers try to include all files located in an archive, so if metadata is stored"] # [doc = " in an archive then it needs to be of a form that the linker is able to process."] # [doc = " Now it's not clear whether metadata still needs to be wrapped into an object file or not."] # [doc = ""] # [doc = " Note, though, that we don't actually want this metadata to show up in any"] # [doc = " final output of the compiler. Instead this is purely for rustc's own"] # [doc = " metadata tracking purposes."] # [doc = ""] # [doc = " With the above in mind, each \"flavor\" of object format gets special"] # [doc = " handling here depending on the target:"] # [doc = ""] # [doc = " * MachO - macos-like targets will insert the metadata into a section that"] # [doc = "   is sort of fake dwarf debug info. Inspecting the source of the macos"] # [doc = "   linker this causes these sections to be skipped automatically because"] # [doc = "   it's not in an allowlist of otherwise well known dwarf section names to"] # [doc = "   go into the final artifact."] # [doc = ""] # [doc = " * WebAssembly - this uses wasm files themselves as the object file format"] # [doc = "   so an empty file with no linking metadata but a single custom section is"] # [doc = "   created holding our metadata."] # [doc = ""] # [doc = " * COFF - Windows-like targets create an object with a section that has"] # [doc = "   the `IMAGE_SCN_LNK_REMOVE` flag set which ensures that if the linker"] # [doc = "   ever sees the section it doesn't process it and it's removed."] # [doc = ""] # [doc = " * ELF - All other targets are similar to Windows in that there's a"] # [doc = "   `SHF_EXCLUDE` flag we can set on sections in an object file to get"] # [doc = "   automatically removed from the final output."] pub (crate) fn create_wrapper_file (sess : & Session , section_name : String , data : & [u8] ,) -> (Vec < u8 > , MetadataPosition) { let Some (mut file) = create_object_file (sess) else { if sess . target . is_like_wasm { return (create_metadata_file_for_wasm (sess , data , & section_name) , MetadataPosition :: First ,) ; } return (data . to_vec () , MetadataPosition :: Last) ; } ; let section = if file . format () == BinaryFormat :: Xcoff { file . add_section (Vec :: new () , b".info" . to_vec () , SectionKind :: Debug) } else { file . add_section (file . segment_name (StandardSegment :: Debug) . to_vec () , section_name . into_bytes () , SectionKind :: Debug ,) } ; match file . format () { BinaryFormat :: Coff => { file . section_mut (section) . flags = SectionFlags :: Coff { characteristics : pe :: IMAGE_SCN_LNK_REMOVE } ; } BinaryFormat :: Elf => { file . section_mut (section) . flags = SectionFlags :: Elf { sh_flags : elf :: SHF_EXCLUDE as u64 } ; } BinaryFormat :: Xcoff => { file . add_section (Vec :: new () , b".text" . to_vec () , SectionKind :: Text) ; file . section_mut (section) . flags = SectionFlags :: Xcoff { s_flags : xcoff :: STYP_INFO as u32 } ; let len : u32 = data . len () . try_into () . unwrap () ; let offset = file . append_section_data (section , & len . to_be_bytes () , 1) ; file . add_symbol (Symbol { name : AIX_METADATA_SYMBOL_NAME . into () , value : offset + 4 , size : 0 , kind : SymbolKind :: Unknown , scope : SymbolScope :: Compilation , weak : false , section : SymbolSection :: Section (section) , flags : SymbolFlags :: Xcoff { n_sclass : xcoff :: C_INFO , x_smtyp : xcoff :: C_HIDEXT , x_smclas : xcoff :: C_HIDEXT , containing_csect : None , } , }) ; } _ => { } } ; file . append_section_data (section , data , 1) ; (file . write () . unwrap () , MetadataPosition :: First) }
}

macro_rules! create_compressed_metadata_file_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function create_compressed_metadata_file in module {}", module_path!());
    };
}

mkfn!{
    create_compressed_metadata_file_introspect!();
    pub fn create_compressed_metadata_file (sess : & Session , metadata : & EncodedMetadata , symbol_name : & str ,) -> Vec < u8 > { let mut packed_metadata = rustc_metadata :: METADATA_HEADER . to_vec () ; packed_metadata . write_all (& (metadata . stub_or_full () . len () as u64) . to_le_bytes ()) . unwrap () ; packed_metadata . extend (metadata . stub_or_full ()) ; let Some (mut file) = create_object_file (sess) else { if sess . target . is_like_wasm { return create_metadata_file_for_wasm (sess , & packed_metadata , ".rustc") ; } return packed_metadata . to_vec () ; } ; if file . format () == BinaryFormat :: Xcoff { return create_compressed_metadata_file_for_xcoff (file , & packed_metadata , symbol_name) ; } let section = file . add_section (file . segment_name (StandardSegment :: Data) . to_vec () , b".rustc" . to_vec () , SectionKind :: ReadOnlyData ,) ; match file . format () { BinaryFormat :: Elf => { file . section_mut (section) . flags = SectionFlags :: Elf { sh_flags : 0 } ; } _ => { } } ; let offset = file . append_section_data (section , & packed_metadata , 1) ; file . add_symbol (Symbol { name : symbol_name . as_bytes () . to_vec () , value : offset , size : packed_metadata . len () as u64 , kind : SymbolKind :: Data , scope : SymbolScope :: Dynamic , weak : false , section : SymbolSection :: Section (section) , flags : SymbolFlags :: None , }) ; file . write () . unwrap () }
}

macro_rules! create_compressed_metadata_file_for_xcoff_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function create_compressed_metadata_file_for_xcoff in module {}", module_path!());
    };
}

mkfn!{
    create_compressed_metadata_file_for_xcoff_introspect!();
    # [doc = " * Xcoff - On AIX, custom sections are merged into predefined sections,"] # [doc = "   so custom .rustc section is not preserved during linking."] # [doc = "   For this reason, we store metadata in predefined .info section, and"] # [doc = "   define a symbol to reference the metadata. To preserve metadata during"] # [doc = "   linking on AIX, we have to"] # [doc = "   1. Create an empty .text section, a empty .data section."] # [doc = "   2. Define an empty symbol named `symbol_name` inside .data section."] # [doc = "   3. Define an symbol named `AIX_METADATA_SYMBOL_NAME` referencing"] # [doc = "      data inside .info section."] # [doc = "   From XCOFF's view, (2) creates a csect entry in the symbol table, the"] # [doc = "   symbol created by (3) is a info symbol for the preceding csect. Thus"] # [doc = "   two symbols are preserved during linking and we can use the second symbol"] # [doc = "   to reference the metadata."] pub fn create_compressed_metadata_file_for_xcoff (mut file : write :: Object < '_ > , data : & [u8] , symbol_name : & str ,) -> Vec < u8 > { assert ! (file . format () == BinaryFormat :: Xcoff) ; file . add_section (Vec :: new () , b".text" . to_vec () , SectionKind :: Text) ; let data_section = file . add_section (Vec :: new () , b".data" . to_vec () , SectionKind :: Data) ; let section = file . add_section (Vec :: new () , b".info" . to_vec () , SectionKind :: Debug) ; file . add_file_symbol ("lib.rmeta" . into ()) ; file . section_mut (section) . flags = SectionFlags :: Xcoff { s_flags : xcoff :: STYP_INFO as u32 } ; file . add_symbol (Symbol { name : symbol_name . as_bytes () . into () , value : 0 , size : 0 , kind : SymbolKind :: Data , scope : SymbolScope :: Dynamic , weak : true , section : SymbolSection :: Section (data_section) , flags : SymbolFlags :: None , }) ; let len : u32 = data . len () . try_into () . unwrap () ; let offset = file . append_section_data (section , & len . to_be_bytes () , 1) ; file . add_symbol (Symbol { name : AIX_METADATA_SYMBOL_NAME . into () , value : offset + 4 , size : 0 , kind : SymbolKind :: Unknown , scope : SymbolScope :: Dynamic , weak : false , section : SymbolSection :: Section (section) , flags : SymbolFlags :: Xcoff { n_sclass : xcoff :: C_INFO , x_smtyp : xcoff :: C_HIDEXT , x_smclas : xcoff :: C_HIDEXT , containing_csect : None , } , }) ; file . append_section_data (section , data , 1) ; file . write () . unwrap () }
}

macro_rules! create_metadata_file_for_wasm_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function create_metadata_file_for_wasm in module {}", module_path!());
    };
}

mkfn!{
    create_metadata_file_for_wasm_introspect!();
    # [doc = " Creates a simple WebAssembly object file, which is itself a wasm module,"] # [doc = " that contains a custom section of the name `section_name` with contents"] # [doc = " `data`."] # [doc = ""] # [doc = " NB: the `object` crate does not yet have support for writing the wasm"] # [doc = " object file format. In lieu of that the `wasm-encoder` crate is used to"] # [doc = " build a wasm file by hand."] # [doc = ""] # [doc = " The wasm object file format is defined at"] # [doc = " <https://github.com/WebAssembly/tool-conventions/blob/main/Linking.md>"] # [doc = " and mainly consists of a `linking` custom section. In this case the custom"] # [doc = " section there is empty except for a version marker indicating what format"] # [doc = " it's in."] # [doc = ""] # [doc = " The main purpose of this is to contain a custom section with `section_name`,"] # [doc = " which is then appended after `linking`."] # [doc = ""] # [doc = " As a further detail the object needs to have a 64-bit memory if `wasm64` is"] # [doc = " the target or otherwise it's interpreted as a 32-bit object which is"] # [doc = " incompatible with 64-bit ones."] pub fn create_metadata_file_for_wasm (sess : & Session , data : & [u8] , section_name : & str) -> Vec < u8 > { assert ! (sess . target . is_like_wasm) ; let mut module = wasm_encoder :: Module :: new () ; let mut imports = wasm_encoder :: ImportSection :: new () ; if sess . target . pointer_width == 64 { imports . import ("env" , "__linear_memory" , wasm_encoder :: MemoryType { minimum : 0 , maximum : None , memory64 : true , shared : false , page_size_log2 : None , } ,) ; } if imports . len () > 0 { module . section (& imports) ; } module . section (& wasm_encoder :: CustomSection { name : "linking" . into () , data : Cow :: Borrowed (& [2]) , }) ; module . section (& wasm_encoder :: CustomSection { name : section_name . into () , data : data . into () }) ; module . finish () }
}