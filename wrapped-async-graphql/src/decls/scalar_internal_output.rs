macro_rules! deps {
    () => {
        ScalarType!();
        MetaType!();
        Result!();
        Registry!();
        ServerResult!();
        Field!();
        OutputType!();
        MetaTypeId!();
        ContextSelectionSet!();
        Scalar!();
    };
}

macro_rules! scalar_internal_output {
    () => {
        deps!();
        # [cfg (not (feature = "boxed-trait"))] # [macro_export] # [doc (hidden)] macro_rules ! scalar_internal_output { ($ ty : ty , $ name : expr , $ desc : expr , $ specified_by_url : expr) => { impl $ crate :: OutputType for $ ty { fn type_name () -> :: std :: borrow :: Cow <'static , :: std :: primitive :: str > { :: std :: borrow :: Cow :: Borrowed ($ name) } fn create_type_info (registry : & mut $ crate :: registry :: Registry ,) -> :: std :: string :: String { registry . create_output_type ::<$ ty , _ > ($ crate :: registry :: MetaTypeId :: Scalar , | _ | { $ crate :: registry :: MetaType :: Scalar { name : :: std :: borrow :: ToOwned :: to_owned ($ name) , description : $ desc , is_valid : :: std :: option :: Option :: Some (:: std :: sync :: Arc :: new (| value | { <$ ty as $ crate :: ScalarType >:: is_valid (value) })) , visible : :: std :: option :: Option :: None , inaccessible : false , tags : :: std :: default :: Default :: default () , specified_by_url : $ specified_by_url , directive_invocations : :: std :: vec :: Vec :: new () , requires_scopes : :: std :: vec :: Vec :: new () , } }) } async fn resolve (& self , _ : &$ crate :: ContextSelectionSet <'_ >, _field : &$ crate :: Positioned <$ crate :: parser :: types :: Field >,) -> $ crate :: ServerResult <$ crate :: Value > { :: std :: result :: Result :: Ok ($ crate :: ScalarType :: to_value (self)) } } } ; }
    };
}

scalar_internal_output!();