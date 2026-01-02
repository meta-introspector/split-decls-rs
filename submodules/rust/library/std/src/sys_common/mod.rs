mkmod!{tests, { 
                getname!(tests);
                getsrc!(tests);
                getpath!(tests);
                get_deps!(tests);
                get_crates!(tests);
                mkinclude!(tests);
                 
            }}
mkmod!{wstr, { 
                getname!(wstr);
                getsrc!(wstr);
                getpath!(wstr);
                get_deps!(wstr);
                get_crates!(wstr);
                mkinclude!(wstr);
                 
            }}
mkitem!{mktrait!{# [doc = " A trait for viewing representations from std types"] # [doc (hidden)] # [allow (dead_code)] pub trait AsInner < Inner : ? Sized > { fn as_inner (& self) -> & Inner ; }}}
mkitem!{mktrait!{# [doc = " A trait for viewing representations from std types"] # [doc (hidden)] # [allow (dead_code)] pub trait AsInnerMut < Inner : ? Sized > { fn as_inner_mut (& mut self) -> & mut Inner ; }}}
mkitem!{mktrait!{# [doc = " A trait for extracting representations from std types"] # [doc (hidden)] pub trait IntoInner < Inner > { fn into_inner (self) -> Inner ; }}}
mkitem!{mktrait!{# [doc = " A trait for creating std types from internal representations"] # [doc (hidden)] pub trait FromInner < Inner > { fn from_inner (inner : Inner) -> Self ; }}}

macro_rules! mul_div_u64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function mul_div_u64 in module {}", module_path!());
    };
}

mkfn!{
    mul_div_u64_introspect!();
    # [allow (dead_code)] pub fn mul_div_u64 (value : u64 , numerator : u64 , denom : u64) -> u64 { let q = value / denom ; let r = value % denom ; q * numerator + r * numerator / denom }
}

macro_rules! ignore_notfound_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function ignore_notfound in module {}", module_path!());
    };
}

mkfn!{
    ignore_notfound_introspect!();
    pub fn ignore_notfound < T > (result : crate :: io :: Result < T >) -> crate :: io :: Result < () > { match result { Err (err) if err . kind () == crate :: io :: ErrorKind :: NotFound => Ok (()) , Ok (_) => Ok (()) , Err (err) => Err (err) , } }
}