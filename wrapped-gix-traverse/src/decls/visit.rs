macro_rules! visit {
    () => {
        # [doc = ""] pub mod visit { # [doc = " What to do after an entry was [recorded][super::Visit::visit_tree()]."] # [derive (Clone , Copy , PartialOrd , PartialEq , Ord , Eq , Hash)] pub enum Action { # [doc = " Continue the traversal of entries."] Continue , # [doc = " Stop the traversal of entries, making this the last call to [`visit_(tree|nontree)(…)`][super::Visit::visit_nontree()]."] Cancel , # [doc = " Don't dive into the entry, skipping children effectively. Only useful in [`visit_tree(…)`][super::Visit::visit_tree()]."] Skip , } impl Action { # [doc = " Returns true if this action means to stop the traversal."] pub fn cancelled (& self) -> bool { matches ! (self , Action :: Cancel) } } }
    };
}

visit!()