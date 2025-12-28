macro_rules! Resource {
    () => {
        type Resource < 's > = ast :: Resource < & 's str > ;
    };
}

Resource!()