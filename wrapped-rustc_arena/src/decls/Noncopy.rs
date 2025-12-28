macro_rules! Noncopy {
    () => {
        # [allow (dead_code)] struct Noncopy { string : String , array : Vec < i32 > , }
    };
}

Noncopy!()