macro_rules! AssertSendSync {
    () => {
        # [allow (dead_code)] trait AssertSendSync : Send + Sync { }
    };
}

AssertSendSync!()