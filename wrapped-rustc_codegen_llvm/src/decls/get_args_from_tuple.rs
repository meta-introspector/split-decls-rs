macro_rules! deps {
    () => {
        PassMode!();
        Builder!();
    };
}

macro_rules! get_args_from_tuple {
    () => {
        deps!();
        fn get_args_from_tuple < 'll , 'tcx > (bx : & mut Builder < '_ , 'll , 'tcx > , tuple_op : OperandRef < 'tcx , & 'll Value > , fn_instance : Instance < 'tcx > ,) -> Vec < & 'll Value > { let cx = bx . cx ; let fn_abi = cx . fn_abi_of_instance (fn_instance , ty :: List :: empty ()) ; match tuple_op . val { OperandValue :: Immediate (val) => vec ! [val] , OperandValue :: Pair (v1 , v2) => vec ! [v1 , v2] , OperandValue :: Ref (ptr) => { let tuple_place = PlaceRef { val : ptr , layout : tuple_op . layout } ; let mut result = Vec :: with_capacity (fn_abi . args . len ()) ; let mut tuple_index = 0 ; for arg in & fn_abi . args { match arg . mode { PassMode :: Ignore => { } PassMode :: Direct (_) | PassMode :: Cast { .. } => { let field = tuple_place . project_field (bx , tuple_index) ; let llvm_ty = field . layout . llvm_type (bx . cx) ; let val = bx . load (llvm_ty , field . val . llval , field . val . align) ; result . push (val) ; tuple_index += 1 ; } PassMode :: Pair (_ , _) => { let field = tuple_place . project_field (bx , tuple_index) ; let llvm_ty = field . layout . llvm_type (bx . cx) ; let pair_val = bx . load (llvm_ty , field . val . llval , field . val . align) ; result . push (bx . extract_value (pair_val , 0)) ; result . push (bx . extract_value (pair_val , 1)) ; tuple_index += 1 ; } PassMode :: Indirect { .. } => { let field = tuple_place . project_field (bx , tuple_index) ; result . push (field . val . llval) ; tuple_index += 1 ; } } } result } OperandValue :: ZeroSized => vec ! [] , } }
    };
}

get_args_from_tuple!()