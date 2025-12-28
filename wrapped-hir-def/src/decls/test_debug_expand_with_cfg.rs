macro_rules! test_debug_expand_with_cfg {
    () => {
        # [test] fn test_debug_expand_with_cfg () { check (r#"
            //- minicore: derive, fmt
            use core::fmt::Debug;

            #[derive(Debug)]
            struct HideAndShow {
                #[cfg(never)]
                always_hide: u32,
                #[cfg(not(never))]
                always_show: u32,
            }
            #[derive(Debug)]
            enum HideAndShowEnum {
                #[cfg(never)]
                AlwaysHide,
                #[cfg(not(never))]
                AlwaysShow{
                    #[cfg(never)]
                    always_hide: u32,
                    #[cfg(not(never))]
                    always_show: u32,
                }
            }
        "# , expect ! [[r#"
use core::fmt::Debug;

#[derive(Debug)]
struct HideAndShow {
    #[cfg(never)]
    always_hide: u32,
    #[cfg(not(never))]
    always_show: u32,
}
#[derive(Debug)]
enum HideAndShowEnum {
    #[cfg(never)]
    AlwaysHide,
    #[cfg(not(never))]
    AlwaysShow{
        #[cfg(never)]
        always_hide: u32,
        #[cfg(not(never))]
        always_show: u32,
    }
}

impl <> $crate::fmt::Debug for HideAndShow< > where {
    fn fmt(&self , f: &mut $crate::fmt::Formatter) -> $crate::fmt::Result {
        match self {
            HideAndShow {
                always_show: always_show,
            }
            =>f.debug_struct("HideAndShow").field("always_show", &always_show).finish()
        }
    }
}
impl <> $crate::fmt::Debug for HideAndShowEnum< > where {
    fn fmt(&self , f: &mut $crate::fmt::Formatter) -> $crate::fmt::Result {
        match self {
            HideAndShowEnum::AlwaysShow {
                always_show: always_show,
            }
            =>f.debug_struct("AlwaysShow").field("always_show", &always_show).finish(),
        }
    }
}"#]] ,) ; }
    };
}

test_debug_expand_with_cfg!()