macro_rules! select_biased {
    () => {
        # [doc = " Selects from a set of channel operations."] # [doc = ""] # [doc = " This macro allows you to define a list of channel operations, wait until any one of them"] # [doc = " becomes ready, and finally execute it. If multiple operations are ready at the same time, the"] # [doc = " operation nearest to the front of the list is always selected (i.e. the biased selection). Use"] # [doc = " [`select!`] for the unbiased selection."] # [doc = ""] # [doc = " Otherwise, this macro's functionality is identical to [`select!`]. Refer to it for the syntax."] # [macro_export] macro_rules ! select_biased { ($ ($ tokens : tt) *) => { { const _IS_BIASED : bool = true ; $ crate :: crossbeam_channel_internal ! ($ ($ tokens) *) } } ; }
    };
}

select_biased!()