// Generated macro for impl_1619 (impl)
macro_rules! Depcrate_build_elfimpl_1619 {
() => {
// Module: crate::build::elf
// Provides: {"impl_1619"}
// Dependencies: {}
impl < 'data , const DYNAMIC : bool > Symbols < 'data , DYNAMIC > { # [doc = " Number of defined symbols."] pub fn count_defined (& self) -> usize { self . into_iter () . filter (| symbol | symbol . st_shndx != elf :: SHN_UNDEF) . count () } # [doc = " Add a new symbol to the table."] pub fn add (& mut self) -> & mut Symbol < 'data , DYNAMIC > { let id = self . next_id () ; self . push (Symbol { id , delete : false , name : ByteString :: default () , section : None , st_info : 0 , st_other : 0 , st_shndx : 0 , st_value : 0 , st_size : 0 , version : VersionId :: local () , version_hidden : false , }) } }
};
}
