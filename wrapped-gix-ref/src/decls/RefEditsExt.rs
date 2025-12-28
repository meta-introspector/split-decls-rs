macro_rules! deps {
    () => {
        PartialNameRef!();
        Error!();
        RefEdit!();
        Target!();
    };
}

macro_rules! RefEditsExt {
    () => {
        deps!();
        # [doc = " An extension trait to perform commonly used operations on edits across different ref stores."] pub trait RefEditsExt < T > where T : std :: borrow :: Borrow < RefEdit > + std :: borrow :: BorrowMut < RefEdit > , { # [doc = " Return true if each ref `name` has exactly one `edit` across multiple ref edits"] fn assure_one_name_has_one_edit (& self) -> Result < () , BString > ; # [doc = " Split all symbolic refs into updates for the symbolic ref as well as all their referents if the `deref` flag is enabled."] # [doc = ""] # [doc = " Note no action is performed if deref isn't specified."] fn extend_with_splits_of_symbolic_refs (& mut self , find : & mut dyn FnMut (& PartialNameRef) -> Option < Target > , make_entry : & mut dyn FnMut (usize , RefEdit) -> T ,) -> Result < () , std :: io :: Error > ; # [doc = " All processing steps in one and in the correct order."] # [doc = ""] # [doc = " Users call this to assure derefs are honored and duplicate checks are done."] fn pre_process (& mut self , find : & mut dyn FnMut (& PartialNameRef) -> Option < Target > , make_entry : & mut dyn FnMut (usize , RefEdit) -> T ,) -> Result < () , std :: io :: Error > { self . extend_with_splits_of_symbolic_refs (find , make_entry) ? ; self . assure_one_name_has_one_edit () . map_err (| name | { std :: io :: Error :: new (std :: io :: ErrorKind :: AlreadyExists , format ! ("A reference named '{name}' has multiple edits") ,) }) } }
    };
}

RefEditsExt!();