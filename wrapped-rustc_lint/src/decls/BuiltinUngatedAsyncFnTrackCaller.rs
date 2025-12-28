macro_rules! BuiltinUngatedAsyncFnTrackCaller {
    () => {
        pub (crate) struct BuiltinUngatedAsyncFnTrackCaller < 'a > { pub label : Span , pub session : & 'a Session , }
    };
}

BuiltinUngatedAsyncFnTrackCaller!()