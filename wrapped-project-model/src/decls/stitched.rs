macro_rules! deps {
    () => {
        ManifestPath!();
    };
}

macro_rules! stitched {
    () => {
        deps!();
        pub (crate) mod stitched { use std :: ops ; use base_db :: CrateName ; use la_arena :: { Arena , Idx } ; use crate :: ManifestPath ; # [derive (Debug , Clone , Eq , PartialEq)] pub struct Stitched { pub (super) crates : Arena < RustLibSrcCrateData > , } impl ops :: Index < RustLibSrcCrate > for Stitched { type Output = RustLibSrcCrateData ; fn index (& self , index : RustLibSrcCrate) -> & RustLibSrcCrateData { & self . crates [index] } } impl Stitched { pub (crate) fn public_deps (& self ,) -> impl Iterator < Item = (CrateName , RustLibSrcCrate , bool) > + '_ { [("core" , true) , ("alloc" , false) , ("std" , true) , ("test" , false)] . into_iter () . filter_map (move | (name , prelude) | { Some ((CrateName :: new (name) . unwrap () , self . by_name (name) ? , prelude)) }) } pub (crate) fn proc_macro (& self) -> Option < RustLibSrcCrate > { self . by_name ("proc_macro") } pub (crate) fn crates (& self) -> impl ExactSizeIterator < Item = RustLibSrcCrate > + '_ { self . crates . iter () . map (| (id , _data) | id) } pub (super) fn by_name (& self , name : & str) -> Option < RustLibSrcCrate > { let (id , _data) = self . crates . iter () . find (| (_id , data) | data . name == name) ? ; Some (id) } } pub (crate) type RustLibSrcCrate = Idx < RustLibSrcCrateData > ; # [derive (Debug , Clone , Eq , PartialEq)] pub (crate) struct RustLibSrcCrateData { pub (crate) name : String , pub (crate) root : ManifestPath , pub (crate) deps : Vec < RustLibSrcCrate > , } pub (super) const SYSROOT_CRATES : & str = "
alloc
backtrace
core
panic_abort
panic_unwind
proc_macro
profiler_builtins
std
stdarch/crates/std_detect
test
unwind" ; pub (super) const ALLOC_DEPS : & str = "core" ; pub (super) const STD_DEPS : & str = "
alloc
panic_unwind
panic_abort
core
profiler_builtins
unwind
std_detect
test" ; pub (super) const PROC_MACRO_DEPS : & str = "
std
core" ; }
    };
}

stitched!();