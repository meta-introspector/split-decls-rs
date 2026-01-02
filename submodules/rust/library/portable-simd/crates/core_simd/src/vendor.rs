mkitem!{# [doc = " Provides implementations of `From<$a> for $b` and `From<$b> for $a` that transmutes the value."] # [allow (unused)] macro_rules ! from_transmute { { unsafe $ a : ty => $ b : ty } => { from_transmute ! { @ impl $ a => $ b } from_transmute ! { @ impl $ b => $ a } } ; { @ impl $ from : ty => $ to : ty } => { impl core :: convert :: From <$ from > for $ to { # [inline] fn from (value : $ from) -> $ to { unsafe { core :: mem :: transmute (value) } } } } ; }}
mkmod!{x86, { 
                getname!(x86);
                getsrc!(x86);
                getpath!(x86);
                get_deps!(x86);
                get_crates!(x86);
                mkinclude!(x86);
                 
            }}
mkmod!{wasm32, { 
                getname!(wasm32);
                getsrc!(wasm32);
                getpath!(wasm32);
                get_deps!(wasm32);
                get_crates!(wasm32);
                mkinclude!(wasm32);
                 
            }}
mkmod!{arm, { 
                getname!(arm);
                getsrc!(arm);
                getpath!(arm);
                get_deps!(arm);
                get_crates!(arm);
                mkinclude!(arm);
                 
            }}
mkmod!{powerpc, { 
                getname!(powerpc);
                getsrc!(powerpc);
                getpath!(powerpc);
                get_deps!(powerpc);
                get_crates!(powerpc);
                mkinclude!(powerpc);
                 
            }}
mkmod!{loongarch64, { 
                getname!(loongarch64);
                getsrc!(loongarch64);
                getpath!(loongarch64);
                get_deps!(loongarch64);
                get_crates!(loongarch64);
                mkinclude!(loongarch64);
                 
            }}