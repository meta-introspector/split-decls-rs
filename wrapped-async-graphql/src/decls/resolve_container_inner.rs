macro_rules! deps {
    () => {
        Fields!();
        ContainerType!();
        ContextSelectionSet!();
        ServerResult!();
    };
}

macro_rules! resolve_container_inner {
    () => {
        deps!();
        async fn resolve_container_inner < 'a , T : ContainerType + ? Sized > (ctx : & ContextSelectionSet < 'a > , root : & 'a T , parallel : bool ,) -> ServerResult < Value > { let mut fields = Fields (Vec :: new ()) ; fields . add_set (ctx , root) ? ; let res = if parallel { futures_util :: future :: try_join_all (fields . 0) . await ? } else { let mut results = Vec :: with_capacity (fields . 0 . len ()) ; for field in fields . 0 { results . push (field . await ?) ; } results } ; Ok (create_value_object (res)) }
    };
}

resolve_container_inner!()