macro_rules! CopiedConsumer {
    () => {
        struct CopiedConsumer < C > { base : C , }
    };
}

CopiedConsumer!()