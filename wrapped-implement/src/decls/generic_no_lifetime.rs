macro_rules! generic_no_lifetime {
    () => {
        # [test] fn generic_no_lifetime () { implement (quote ! (IAsyncOperationWithProgress < T , P >, IAsyncInfo) , quote ! { struct OperationWithProgress < T , P > (SyncState < IAsyncOperationWithProgress < T , P >>) where T : RuntimeType + 'static , P : RuntimeType + 'static ; } ,) ; }
    };
}

generic_no_lifetime!();