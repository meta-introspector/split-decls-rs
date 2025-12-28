macro_rules! deps {
    () => {
        ContainerType!();
        ServerResult!();
        ContextSelectionSet!();
    };
}

macro_rules! resolve_container_serial {
    () => {
        deps!();
        # [doc = " Resolve an container by executing each of the fields serially."] pub async fn resolve_container_serial < 'a , T : ContainerType + ? Sized > (ctx : & ContextSelectionSet < 'a > , root : & 'a T ,) -> ServerResult < Value > { resolve_container_inner (ctx , root , false) . await }
    };
}

resolve_container_serial!();