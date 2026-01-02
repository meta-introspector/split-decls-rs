mkuse!{use core :: ptr ;}
mkuse!{use super :: DwarfReader ;}
mkitem!{pub const DW_EH_PE_omit : u8 = 0xFF ;}
mkitem!{pub const DW_EH_PE_absptr : u8 = 0x00 ;}
mkitem!{pub const DW_EH_PE_uleb128 : u8 = 0x01 ;}
mkitem!{pub const DW_EH_PE_udata2 : u8 = 0x02 ;}
mkitem!{pub const DW_EH_PE_udata4 : u8 = 0x03 ;}
mkitem!{pub const DW_EH_PE_udata8 : u8 = 0x04 ;}
mkitem!{pub const DW_EH_PE_sleb128 : u8 = 0x09 ;}
mkitem!{pub const DW_EH_PE_sdata2 : u8 = 0x0A ;}
mkitem!{pub const DW_EH_PE_sdata4 : u8 = 0x0B ;}
mkitem!{pub const DW_EH_PE_sdata8 : u8 = 0x0C ;}
mkitem!{pub const DW_EH_PE_pcrel : u8 = 0x10 ;}
mkitem!{pub const DW_EH_PE_textrel : u8 = 0x20 ;}
mkitem!{pub const DW_EH_PE_datarel : u8 = 0x30 ;}
mkitem!{pub const DW_EH_PE_funcrel : u8 = 0x40 ;}
mkitem!{pub const DW_EH_PE_aligned : u8 = 0x50 ;}
mkitem!{pub const DW_EH_PE_indirect : u8 = 0x80 ;}
mkitem!{mkstruct!{# [derive (Copy , Clone)] pub struct EHContext < 'a > { pub ip : * const u8 , pub func_start : * const u8 , pub get_text_start : & 'a dyn Fn () -> * const u8 , pub get_data_start : & 'a dyn Fn () -> * const u8 , }}}
mkitem!{# [doc = " Landing pad."] type LPad = * const u8 ;}
mkitem!{mkenum!{pub enum EHAction { None , Cleanup (LPad) , Catch (LPad) , Filter (LPad) , Terminate , }}}
mkitem!{# [doc = " 32-bit ARM Darwin platforms uses SjLj exceptions."] # [doc = ""] # [doc = " The exception is watchOS armv7k (specifically that subarchitecture), which"] # [doc = " instead uses DWARF Call Frame Information (CFI) unwinding."] # [doc = ""] # [doc = " <https://github.com/llvm/llvm-project/blob/llvmorg-18.1.4/clang/lib/Driver/ToolChains/Darwin.cpp#L3107-L3119>"] pub const USING_SJLJ_EXCEPTIONS : bool = cfg ! (all (target_vendor = "apple" , not (target_os = "watchos") , target_arch = "arm")) ;}

macro_rules! find_eh_action_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function find_eh_action in module {}", module_path!());
    };
}

mkfn!{
    find_eh_action_introspect!();
    pub unsafe fn find_eh_action (lsda : * const u8 , context : & EHContext < '_ >) -> Result < EHAction , () > { if lsda . is_null () { return Ok (EHAction :: None) ; } let func_start = context . func_start ; let mut reader = DwarfReader :: new (lsda) ; let lpad_base = unsafe { let start_encoding = reader . read :: < u8 > () ; if start_encoding != DW_EH_PE_omit { read_encoded_pointer (& mut reader , context , start_encoding) ? } else { func_start } } ; let call_site_encoding = unsafe { let ttype_encoding = reader . read :: < u8 > () ; if ttype_encoding != DW_EH_PE_omit { reader . read_uleb128 () ; } reader . read :: < u8 > () } ; let action_table = unsafe { let call_site_table_length = reader . read_uleb128 () ; reader . ptr . add (call_site_table_length as usize) } ; let ip = context . ip ; if ! USING_SJLJ_EXCEPTIONS { while reader . ptr < action_table { unsafe { let cs_start = read_encoded_offset (& mut reader , call_site_encoding) ? ; let cs_len = read_encoded_offset (& mut reader , call_site_encoding) ? ; let cs_lpad = read_encoded_offset (& mut reader , call_site_encoding) ? ; let cs_action_entry = reader . read_uleb128 () ; if ip < func_start . wrapping_add (cs_start) { break ; } if ip < func_start . wrapping_add (cs_start + cs_len) { if cs_lpad == 0 { return Ok (EHAction :: None) ; } else { let lpad = lpad_base . wrapping_add (cs_lpad) ; return Ok (interpret_cs_action (action_table , cs_action_entry , lpad)) ; } } } } Ok (EHAction :: Terminate) } else { match ip . addr () as isize { - 1 => return Ok (EHAction :: None) , 0 => return Ok (EHAction :: Terminate) , _ => () , } let mut idx = ip . addr () ; loop { let cs_lpad = unsafe { reader . read_uleb128 () } ; let cs_action_entry = unsafe { reader . read_uleb128 () } ; idx -= 1 ; if idx == 0 { let lpad = ptr :: with_exposed_provenance ((cs_lpad + 1) as usize) ; return Ok (unsafe { interpret_cs_action (action_table , cs_action_entry , lpad) }) ; } } } }
}

macro_rules! interpret_cs_action_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function interpret_cs_action in module {}", module_path!());
    };
}

mkfn!{
    interpret_cs_action_introspect!();
    unsafe fn interpret_cs_action (action_table : * const u8 , cs_action_entry : u64 , lpad : LPad ,) -> EHAction { if cs_action_entry == 0 { EHAction :: Cleanup (lpad) } else { let action_record = unsafe { action_table . offset (cs_action_entry as isize - 1) } ; let mut action_reader = DwarfReader :: new (action_record) ; let ttype_index = unsafe { action_reader . read_sleb128 () } ; if ttype_index == 0 { EHAction :: Cleanup (lpad) } else if ttype_index > 0 { EHAction :: Catch (lpad) } else { EHAction :: Filter (lpad) } } }
}

macro_rules! round_up_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function round_up in module {}", module_path!());
    };
}

mkfn!{
    round_up_introspect!();
    # [inline] fn round_up (unrounded : usize , align : usize) -> Result < usize , () > { if align . is_power_of_two () { Ok ((unrounded + align - 1) & ! (align - 1)) } else { Err (()) } }
}

macro_rules! read_encoded_offset_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function read_encoded_offset in module {}", module_path!());
    };
}

mkfn!{
    read_encoded_offset_introspect!();
    # [doc = " Reads an offset (`usize`) from `reader` whose encoding is described by `encoding`."] # [doc = ""] # [doc = " `encoding` must be a [DWARF Exception Header Encoding as described by the LSB spec][LSB-dwarf-ext]."] # [doc = " In addition the upper (\"application\") part must be zero."] # [doc = ""] # [doc = " # Errors"] # [doc = " Returns `Err` if `encoding`"] # [doc = " * is not a valid DWARF Exception Header Encoding,"] # [doc = " * is `DW_EH_PE_omit`, or"] # [doc = " * has a non-zero application part."] # [doc = ""] # [doc = " [LSB-dwarf-ext]: https://refspecs.linuxfoundation.org/LSB_5.0.0/LSB-Core-generic/LSB-Core-generic/dwarfext.html"] unsafe fn read_encoded_offset (reader : & mut DwarfReader , encoding : u8) -> Result < usize , () > { if encoding == DW_EH_PE_omit || encoding & 0xF0 != 0 { return Err (()) ; } let result = unsafe { match encoding & 0x0F { DW_EH_PE_absptr => reader . read :: < usize > () , DW_EH_PE_uleb128 => reader . read_uleb128 () as usize , DW_EH_PE_udata2 => reader . read :: < u16 > () as usize , DW_EH_PE_udata4 => reader . read :: < u32 > () as usize , DW_EH_PE_udata8 => reader . read :: < u64 > () as usize , DW_EH_PE_sleb128 => reader . read_sleb128 () as usize , DW_EH_PE_sdata2 => reader . read :: < i16 > () as usize , DW_EH_PE_sdata4 => reader . read :: < i32 > () as usize , DW_EH_PE_sdata8 => reader . read :: < i64 > () as usize , _ => return Err (()) , } } ; Ok (result) }
}

macro_rules! read_encoded_pointer_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function read_encoded_pointer in module {}", module_path!());
    };
}

mkfn!{
    read_encoded_pointer_introspect!();
    # [doc = " Reads a pointer from `reader` whose encoding is described by `encoding`."] # [doc = ""] # [doc = " `encoding` must be a [DWARF Exception Header Encoding as described by the LSB spec][LSB-dwarf-ext]."] # [doc = ""] # [doc = " # Errors"] # [doc = " Returns `Err` if `encoding`"] # [doc = " * is not a valid DWARF Exception Header Encoding,"] # [doc = " * is `DW_EH_PE_omit`, or"] # [doc = " * combines `DW_EH_PE_absptr` or `DW_EH_PE_aligned` application part with an integer encoding"] # [doc = "   (not `DW_EH_PE_absptr`) in the value format part."] # [doc = ""] # [doc = " [LSB-dwarf-ext]: https://refspecs.linuxfoundation.org/LSB_5.0.0/LSB-Core-generic/LSB-Core-generic/dwarfext.html"] unsafe fn read_encoded_pointer (reader : & mut DwarfReader , context : & EHContext < '_ > , encoding : u8 ,) -> Result < * const u8 , () > { if encoding == DW_EH_PE_omit { return Err (()) ; } let base_ptr = match encoding & 0x70 { DW_EH_PE_absptr => core :: ptr :: null () , DW_EH_PE_pcrel => reader . ptr , DW_EH_PE_funcrel => { if context . func_start . is_null () { return Err (()) ; } context . func_start } DW_EH_PE_textrel => (* context . get_text_start) () , DW_EH_PE_datarel => (* context . get_data_start) () , DW_EH_PE_aligned => { reader . ptr = reader . ptr . with_addr (round_up (reader . ptr . addr () , size_of :: < * const u8 > ()) ?) ; core :: ptr :: null () } _ => return Err (()) , } ; let mut ptr = if base_ptr . is_null () { if encoding & 0x0F != DW_EH_PE_absptr { return Err (()) ; } unsafe { reader . read :: < * const u8 > () } } else { let offset = unsafe { read_encoded_offset (reader , encoding & 0x0F) ? } ; base_ptr . wrapping_add (offset) } ; if encoding & DW_EH_PE_indirect != 0 { ptr = unsafe { * (ptr . cast :: < * const u8 > ()) } ; } Ok (ptr) }
}