macro_rules! coroutine_kind_label {
    () => {
        fn coroutine_kind_label (coroutine_kind : Option < CoroutineKind >) -> & 'static str { use CoroutineDesugaring :: * ; use CoroutineKind :: * ; use CoroutineSource :: * ; match coroutine_kind { Some (Desugared (Gen , Block)) => "gen_block" , Some (Desugared (Gen , Closure)) => "gen_closure" , Some (Desugared (Gen , Fn)) => "gen_fn" , Some (Desugared (Async , Block)) => "async_block" , Some (Desugared (Async , Closure)) => "async_closure" , Some (Desugared (Async , Fn)) => "async_fn" , Some (Desugared (AsyncGen , Block)) => "async_gen_block" , Some (Desugared (AsyncGen , Closure)) => "async_gen_closure" , Some (Desugared (AsyncGen , Fn)) => "async_gen_fn" , Some (Coroutine (_)) => "coroutine" , None => "closure" , } }
    };
}

coroutine_kind_label!();