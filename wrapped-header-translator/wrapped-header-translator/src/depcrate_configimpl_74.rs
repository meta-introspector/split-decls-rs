// Generated macro for impl_74 (impl)
macro_rules! Depcrate_configimpl_74 {
() => {
// Module: crate::config
// Provides: {"impl_74"}
// Dependencies: {}
impl FromStr for ItemGeneric { type Err = Box < dyn Error > ; fn from_str (s : & str) -> Result < Self , Self :: Err > { if let Some ((id , generics)) = s . rsplit_once ('<') { let (generics , rest) = generics . rsplit_once ('>') . ok_or_else (| | std :: io :: Error :: other ("missing closing >")) ? ; if ! rest . is_empty () { Err (std :: io :: Error :: other ("unexpected after >")) ? ; } Ok (Self { id : ItemIdentifier :: from_str (id) ? , generics : generics . split (",") . map (| generic | generic . trim () . parse :: < ItemGeneric > ()) . collect :: < Result < Vec < _ > , _ > > () ? , }) } else { Ok (Self { id : ItemIdentifier :: from_str (s) ? , generics : vec ! [] , }) } } }
};
}
