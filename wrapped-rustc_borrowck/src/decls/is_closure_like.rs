macro_rules! is_closure_like {
    () => {
        # [doc = " If the type is a `Coroutine`, `Closure`, or `CoroutineClosure`"] fn is_closure_like (ty : Ty < '_ >) -> bool { ty . is_closure () || ty . is_coroutine () || ty . is_coroutine_closure () }
    };
}

is_closure_like!();