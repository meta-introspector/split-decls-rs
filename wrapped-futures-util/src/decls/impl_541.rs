macro_rules! impl_541 {
    () => {
        impl < St , Fc > FlattenUnorderedWithFlowControllerProj < '_ , St , Fc > where St : Stream , { # [doc = " Checks if current `inner_streams` bucket size is greater than optional limit."] fn is_exceeded_limit (& self) -> bool { self . limit . map_or (false , | limit | self . inner_streams . len () >= limit . get ()) } }
    };
}

impl_541!()