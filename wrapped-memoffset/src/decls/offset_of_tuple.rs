macro_rules! offset_of_tuple {
    () => {
        # [doc = " Calculates the offset of the specified field from the start of the tuple."] # [doc = ""] # [doc = " ## Examples"] # [doc = " ```"] # [doc = " use memoffset::offset_of_tuple;"] # [doc = ""] # [doc = " assert!(offset_of_tuple!((u8, u32), 1) >= 0, \"Tuples do not have a defined layout\");"] # [doc = " ```"] # [cfg (tuple_ty)] # [macro_export (local_inner_macros)] macro_rules ! offset_of_tuple { ($ parent : ty , $ field : tt) => { { _memoffset__offset_of_tuple_impl ! ($ parent , $ field) } } ; }
    };
}

offset_of_tuple!()