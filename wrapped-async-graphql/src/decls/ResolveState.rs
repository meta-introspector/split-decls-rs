macro_rules! ResolveState {
    () => {
        struct ResolveState { path : Vec < String > , field_name : String , parent_type : String , return_type : String , start_time : DateTime < Utc > , end_time : DateTime < Utc > , start_offset : i64 , }
    };
}

ResolveState!();