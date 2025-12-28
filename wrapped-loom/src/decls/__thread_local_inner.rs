macro_rules! __thread_local_inner {
    () => {
        # [macro_export] # [doc (hidden)] macro_rules ! __thread_local_inner { ($ (# [$ attr : meta]) * $ vis : vis $ name : ident , $ t : ty , $ init : expr) => { $ (# [$ attr]) * $ vis static $ name : $ crate :: thread :: LocalKey <$ t > = $ crate :: thread :: LocalKey { init : (|| { $ init }) as fn () -> $ t , _p : std :: marker :: PhantomData , } ; } ; }
    };
}

__thread_local_inner!()