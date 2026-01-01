/* FP:memmap.rs-0001 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_memmap_USE_0001
/* FP:memmap.rs-0002 */ use std :: fs :: File ;
/* FP:memmap.rs-0003 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_memmap_USE_0002
/* FP:memmap.rs-0004 */ use std :: io ;
/* FP:memmap.rs-0005 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_memmap_USE_0003
/* FP:memmap.rs-0006 */ use std :: ops :: { Deref , DerefMut } ;
/* FP:memmap.rs-0007 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_memmap_STRUCT_0004
/* FP:memmap.rs-0008 */ # [doc = " A trivial wrapper for [`memmap2::Mmap`] (or `Vec<u8>` on WASM)."] # [cfg (not (any (miri , target_arch = "wasm32")))] pub struct Mmap (memmap2 :: Mmap) ;
/* FP:memmap.rs-0009 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_memmap_STRUCT_0005
/* FP:memmap.rs-0010 */ # [cfg (any (miri , target_arch = "wasm32"))] pub struct Mmap (Vec < u8 >) ;
/* FP:memmap.rs-0011 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_memmap_IMPL_0006
/* FP:memmap.rs-0012 */ # [cfg (not (any (miri , target_arch = "wasm32")))] impl Mmap { # [doc = " # Safety"] # [doc = ""] # [doc = " The given file must not be mutated (i.e., not written, not truncated, ...) until the mapping is closed."] # [doc = ""] # [doc = " However in practice most callers do not ensure this, so uses of this function are likely unsound."] # [inline] pub unsafe fn map (file : File) -> io :: Result < Self > { unsafe { memmap2 :: MmapOptions :: new () . map_copy_read_only (& file) . map (Mmap) } } }
/* FP:memmap.rs-0013 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_memmap_IMPL_0007
/* FP:memmap.rs-0014 */ # [cfg (any (miri , target_arch = "wasm32"))] impl Mmap { # [inline] pub unsafe fn map (mut file : File) -> io :: Result < Self > { use std :: io :: Read ; let mut data = Vec :: new () ; file . read_to_end (& mut data) ? ; Ok (Mmap (data)) } }
/* FP:memmap.rs-0015 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_memmap_IMPL_0008
/* FP:memmap.rs-0016 */ impl Deref for Mmap { type Target = [u8] ; # [inline] fn deref (& self) -> & [u8] { & self . 0 } }
/* FP:memmap.rs-0017 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_memmap_IMPL_0009
/* FP:memmap.rs-0018 */ impl AsRef < [u8] > for Mmap { fn as_ref (& self) -> & [u8] { & self . 0 } }
/* FP:memmap.rs-0019 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_memmap_STRUCT_0010
/* FP:memmap.rs-0020 */ # [cfg (not (any (miri , target_arch = "wasm32")))] pub struct MmapMut (memmap2 :: MmapMut) ;
/* FP:memmap.rs-0021 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_memmap_STRUCT_0011
/* FP:memmap.rs-0022 */ # [cfg (any (miri , target_arch = "wasm32"))] pub struct MmapMut (Vec < u8 >) ;
/* FP:memmap.rs-0023 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_memmap_IMPL_0012
/* FP:memmap.rs-0024 */ # [cfg (not (any (miri , target_arch = "wasm32")))] impl MmapMut { # [inline] pub fn map_anon (len : usize) -> io :: Result < Self > { let mmap = memmap2 :: MmapMut :: map_anon (len) ? ; Ok (MmapMut (mmap)) } # [inline] pub fn flush (& mut self) -> io :: Result < () > { self . 0 . flush () } # [inline] pub fn make_read_only (self) -> std :: io :: Result < Mmap > { let mmap = self . 0 . make_read_only () ? ; Ok (Mmap (mmap)) } }
/* FP:memmap.rs-0025 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_memmap_IMPL_0013
/* FP:memmap.rs-0026 */ # [cfg (any (miri , target_arch = "wasm32"))] impl MmapMut { # [inline] pub fn map_anon (len : usize) -> io :: Result < Self > { let data = Vec :: with_capacity (len) ; Ok (MmapMut (data)) } # [inline] pub fn flush (& mut self) -> io :: Result < () > { Ok (()) } # [inline] pub fn make_read_only (self) -> std :: io :: Result < Mmap > { Ok (Mmap (self . 0)) } }
/* FP:memmap.rs-0027 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_memmap_IMPL_0014
/* FP:memmap.rs-0028 */ impl Deref for MmapMut { type Target = [u8] ; # [inline] fn deref (& self) -> & [u8] { & self . 0 } }
/* FP:memmap.rs-0029 */ #[warn(unused_variables)] // AST_.._rust_compiler_rustc_data_structures_src_memmap_IMPL_0015
/* FP:memmap.rs-0030 */ impl DerefMut for MmapMut { # [inline] fn deref_mut (& mut self) -> & mut [u8] { & mut self . 0 } }