macro_rules! deps {
    () => {
        CodegenCx!();
        LlvmType!();
    };
}

macro_rules! impl_7 {
    () => {
        deps!();
        impl LlvmType for CastTarget { fn llvm_type < 'll > (& self , cx : & CodegenCx < 'll , '_ >) -> & 'll Type { let rest_ll_unit = self . rest . unit . llvm_type (cx) ; let rest_count = if self . rest . total == Size :: ZERO { 0 } else { assert_ne ! (self . rest . unit . size , Size :: ZERO , "total size {:?} cannot be divided into units of zero size" , self . rest . total) ; if ! self . rest . total . bytes () . is_multiple_of (self . rest . unit . size . bytes ()) { assert_eq ! (self . rest . unit . kind , RegKind :: Integer , "only int regs can be split") ; } self . rest . total . bytes () . div_ceil (self . rest . unit . size . bytes ()) } ; if self . prefix . iter () . all (| x | x . is_none ()) { if rest_count == 1 && (! self . rest . is_consecutive || self . rest . unit != Reg :: i128 ()) { return rest_ll_unit ; } return cx . type_array (rest_ll_unit , rest_count) ; } let prefix_args = self . prefix . iter () . flat_map (| option_reg | option_reg . map (| reg | reg . llvm_type (cx))) ; let rest_args = (0 .. rest_count) . map (| _ | rest_ll_unit) ; let args : Vec < _ > = prefix_args . chain (rest_args) . collect () ; cx . type_struct (& args , false) } }
    };
}

impl_7!()