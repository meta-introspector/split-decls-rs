macro_rules! deps {
    () => {
        Either!();
        Offset!();
        UnblamedHunk!();
    };
}

macro_rules! impl_42 {
    () => {
        deps!();
        impl UnblamedHunk { fn shift_by (mut self , suspect : ObjectId , offset : Offset) -> Self { if let Some (entry) = self . suspects . iter_mut () . find (| entry | entry . 0 == suspect) { entry . 1 = entry . 1 . shift_by (offset) ; } self } fn split_at (self , suspect : ObjectId , line_number_in_destination : u32) -> Either < Self , (Self , Self) > { match self . get_range (& suspect) { None => Either :: Left (self) , Some (range_in_suspect) => { if ! range_in_suspect . contains (& line_number_in_destination) { return Either :: Left (self) ; } let split_at_from_start = line_number_in_destination - range_in_suspect . start ; if split_at_from_start > 0 { let new_suspects_before = self . suspects . iter () . map (| (suspect , range) | (* suspect , range . start .. (range . start + split_at_from_start))) ; let new_suspects_after = self . suspects . iter () . map (| (suspect , range) | (* suspect , (range . start + split_at_from_start) .. range . end)) ; let new_hunk_before = Self { range_in_blamed_file : self . range_in_blamed_file . start .. (self . range_in_blamed_file . start + split_at_from_start) , suspects : new_suspects_before . collect () , source_file_name : self . source_file_name . clone () , } ; let new_hunk_after = Self { range_in_blamed_file : (self . range_in_blamed_file . start + split_at_from_start) .. (self . range_in_blamed_file . end) , suspects : new_suspects_after . collect () , source_file_name : self . source_file_name , } ; Either :: Right ((new_hunk_before , new_hunk_after)) } else { Either :: Left (self) } } } } # [doc = " This is like [`Self::pass_blame()`], but easier to use in places where the 'passing' is"] # [doc = " done 'inline'."] fn passed_blame (mut self , from : ObjectId , to : ObjectId) -> Self { if let Some (entry) = self . suspects . iter_mut () . find (| entry | entry . 0 == from) { entry . 0 = to ; } self } # [doc = " Transfer all ranges from the commit at `from` to the commit at `to`."] fn pass_blame (& mut self , from : ObjectId , to : ObjectId) { if let Some (entry) = self . suspects . iter_mut () . find (| entry | entry . 0 == from) { entry . 0 = to ; } } fn clone_blame (& mut self , from : ObjectId , to : ObjectId) { if let Some (range_in_suspect) = self . get_range (& from) { self . suspects . push ((to , range_in_suspect . clone ())) ; } } fn remove_blame (& mut self , suspect : ObjectId) { self . suspects . retain (| entry | entry . 0 != suspect) ; } }
    };
}

impl_42!()