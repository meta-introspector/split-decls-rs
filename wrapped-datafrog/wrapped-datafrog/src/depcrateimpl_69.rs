// Generated macro for impl_69 (impl)
macro_rules! Depcrateimpl_69 {
() => {
// Module: crate
// Provides: {"impl_69"}
// Dependencies: {}
impl < Tuple : Ord > VariableTrait for Variable < Tuple > { fn changed (& mut self) -> bool { if ! self . recent . borrow () . is_empty () { let mut recent = :: std :: mem :: replace (& mut (* self . recent . borrow_mut ()) , Vec :: new () . into ()) ; while self . stable . borrow () . last () . map (| x | x . len () <= 2 * recent . len ()) == Some (true) { let last = self . stable . borrow_mut () . pop () . unwrap () ; recent = recent . merge (last) ; } self . stable . borrow_mut () . push (recent) ; } let to_add = self . to_add . borrow_mut () . pop () ; if let Some (mut to_add) = to_add { while let Some (to_add_more) = self . to_add . borrow_mut () . pop () { to_add = to_add . merge (to_add_more) ; } if self . distinct { for batch in self . stable . borrow () . iter () { let mut slice = & batch [..] ; if slice . len () > 4 * to_add . elements . len () { to_add . elements . retain (| x | { slice = join :: gallop (slice , | y | y < x) ; slice . is_empty () || & slice [0] != x }) ; } else { to_add . elements . retain (| x | { while ! slice . is_empty () && & slice [0] < x { slice = & slice [1 ..] ; } slice . is_empty () || & slice [0] != x }) ; } } } * self . recent . borrow_mut () = to_add ; } ! self . recent . borrow () . is_empty () } }
};
}
