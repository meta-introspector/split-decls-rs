macro_rules! deps {
    () => {
        AllocatorKind!();
    };
}

macro_rules! alloc_error_handler_name {
    () => {
        deps!();
        pub fn alloc_error_handler_name (alloc_error_handler_kind : AllocatorKind) -> & 'static str { match alloc_error_handler_kind { AllocatorKind :: Global => "__rg_oom" , AllocatorKind :: Default => "__rdl_oom" , } }
    };
}

alloc_error_handler_name!()