macro_rules! deps {
    () => {
        ServerResult!();
        ContainerType!();
        ContextSelectionSet!();
    };
}

macro_rules! resolve_container {
    () => {
        deps!();
        # [doc = " Resolve an container by executing each of the fields concurrently."] pub async fn resolve_container < 'a , T : ContainerType + ? Sized > (ctx : & ContextSelectionSet < 'a > , root : & 'a T ,) -> ServerResult < Value > { resolve_container_inner (ctx , root , true) . await }
    };
}

resolve_container!()