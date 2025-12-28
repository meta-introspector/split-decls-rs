macro_rules! deps {
    () => {
        Visit!();
        Action!();
        Delegate!();
        Change!();
        Recorder!();
        Location!();
    };
}

macro_rules! tree_to_changes {
    () => {
        deps!();
        mod tree_to_changes { use bstr :: BStr ; use gix_object :: tree :: EntryRef ; use crate :: tree :: visit :: Change ; pub struct Delegate < 'a > { push : & 'a mut dyn FnMut (Change , & BStr) , recorder : gix_traverse :: tree :: Recorder , } impl < 'a > Delegate < 'a > { pub fn new (push : & 'a mut dyn FnMut (Change , & BStr) , location : Option < crate :: tree :: recorder :: Location >) -> Self { let location = location . map (| t | match t { crate :: tree :: recorder :: Location :: FileName => gix_traverse :: tree :: recorder :: Location :: FileName , crate :: tree :: recorder :: Location :: Path => gix_traverse :: tree :: recorder :: Location :: Path , }) ; Self { push , recorder : gix_traverse :: tree :: Recorder :: default () . track_location (location) , } } } impl gix_traverse :: tree :: Visit for Delegate < '_ > { fn pop_back_tracked_path_and_set_current (& mut self) { self . recorder . pop_back_tracked_path_and_set_current () ; } fn pop_front_tracked_path_and_set_current (& mut self) { self . recorder . pop_front_tracked_path_and_set_current () ; } fn push_back_tracked_path_component (& mut self , component : & BStr) { self . recorder . push_back_tracked_path_component (component) ; } fn push_path_component (& mut self , component : & BStr) { self . recorder . push_path_component (component) ; } fn pop_path_component (& mut self) { self . recorder . pop_path_component () ; } fn visit_tree (& mut self , _entry : & EntryRef < '_ >) -> gix_traverse :: tree :: visit :: Action { gix_traverse :: tree :: visit :: Action :: Continue } fn visit_nontree (& mut self , entry : & EntryRef < '_ >) -> gix_traverse :: tree :: visit :: Action { if entry . mode . is_blob () { (self . push) (Change :: Modification { previous_entry_mode : entry . mode , previous_oid : gix_hash :: ObjectId :: null (entry . oid . kind ()) , entry_mode : entry . mode , oid : entry . oid . to_owned () , } , self . recorder . path () ,) ; } gix_traverse :: tree :: visit :: Action :: Continue } } }
    };
}

tree_to_changes!();