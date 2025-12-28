macro_rules! deps {
    () => {
        EncodedStringRef!();
        Entry!();
        NameEntry!();
        EmailEntry!();
    };
}

macro_rules! impl_28 {
    () => {
        deps!();
        impl EmailEntry { pub fn merge (& mut self , crate :: Entry { new_name , new_email , old_name , old_email : _ , } : crate :: Entry < '_ > ,) { let new_email = new_email . map (ToOwned :: to_owned) ; let new_name = new_name . map (ToOwned :: to_owned) ; match old_name { None => { self . new_email = new_email ; self . new_name = new_name ; } Some (old_name) => { let old_name : EncodedStringRef < '_ > = old_name . into () ; match self . entries_by_old_name . binary_search_by (| e | e . old_name . cmp_ref (old_name)) { Ok (pos) => { let entry = & mut self . entries_by_old_name [pos] ; entry . new_name = new_name ; entry . new_email = new_email ; } Err (insert_pos) => self . entries_by_old_name . insert (insert_pos , NameEntry { new_name , new_email , old_name : old_name . into () , } ,) , } } } } }
    };
}

impl_28!()