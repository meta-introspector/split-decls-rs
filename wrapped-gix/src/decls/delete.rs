macro_rules! deps {
    () => {
        Error!();
        Change!();
        Note!();
        Reference!();
    };
}

macro_rules! delete {
    () => {
        deps!();
        # [doc = ""] pub mod delete { use gix_ref :: transaction :: { Change , PreviousValue , RefEdit , RefLog } ; use crate :: Reference ; impl Reference < '_ > { # [doc = " Delete this reference or fail if it was changed since last observed."] # [doc = " Note that this instance remains available in memory but probably shouldn't be used anymore."] pub fn delete (& self) -> Result < () , crate :: reference :: edit :: Error > { self . repo . edit_reference (RefEdit { change : Change :: Delete { expected : PreviousValue :: MustExistAndMatch (self . inner . target . clone ()) , log : RefLog :: AndReference , } , name : self . inner . name . clone () , deref : false , }) . map (| _ | ()) } } }
    };
}

delete!()