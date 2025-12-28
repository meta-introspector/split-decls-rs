macro_rules! deps {
    () => {
        FieldInfo!();
        UnsizedFields!();
    };
}

macro_rules! make_encode_impl {
    () => {
        deps!();
        fn make_encode_impl (sized_fields : & [FieldInfo] , unsized_field_info : & UnsizedFields , name : & Ident , ule_name : & Ident , maybe_lt_bound : & Option < TokenStream2 > ,) -> TokenStream2 { let mut lengths = vec ! [] ; for field in sized_fields { let ty = & field . field . ty ; lengths . push (quote ! (:: core :: mem :: size_of ::<<# ty as zerovec :: ule :: AsULE >:: ULE > ())) ; } let (encoders , remaining_offset) = utils :: generate_per_field_offsets (sized_fields , true , | field , prev_offset_ident , size_ident | { let ty = & field . field . ty ; let accessor = & field . accessor ; quote ! (# [expect (clippy :: indexing_slicing)] let out = & mut dst [# prev_offset_ident .. # prev_offset_ident + # size_ident] ; let unaligned = zerovec :: ule :: AsULE :: to_unaligned (self .# accessor) ; let unaligned_slice = & [unaligned] ; let src = <<# ty as zerovec :: ule :: AsULE >:: ULE as zerovec :: ule :: ULE >:: slice_as_bytes (unaligned_slice) ; out . copy_from_slice (src) ;) } ,) ; let last_encode_len = unsized_field_info . encode_len () ; let last_encode_write = unsized_field_info . encode_write (quote ! (out)) ; quote ! (unsafe impl # maybe_lt_bound zerovec :: ule :: EncodeAsVarULE <# ule_name > for # name # maybe_lt_bound { fn encode_var_ule_as_slices < R > (& self , cb : impl FnOnce (& [& [u8]]) -> R) -> R { unreachable ! ("other two methods implemented") } fn encode_var_ule_len (& self) -> usize { # (# lengths +) * # last_encode_len } fn encode_var_ule_write (& self , mut dst : & mut [u8]) { debug_assert_eq ! (self . encode_var_ule_len () , dst . len ()) ; # encoders # [expect (clippy :: indexing_slicing)] let out = & mut dst [# remaining_offset ..] ; # last_encode_write } } unsafe impl # maybe_lt_bound zerovec :: ule :: EncodeAsVarULE <# ule_name > for &'_ # name # maybe_lt_bound { fn encode_var_ule_as_slices < R > (& self , cb : impl FnOnce (& [& [u8]]) -> R) -> R { unreachable ! ("other two methods implemented") } fn encode_var_ule_len (& self) -> usize { (** self) . encode_var_ule_len () } fn encode_var_ule_write (& self , mut dst : & mut [u8]) { (** self) . encode_var_ule_write (dst) } }) }
    };
}

make_encode_impl!()