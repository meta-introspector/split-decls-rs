macro_rules! TestType {
    () => {
        enum TestType { UnitTest , IntegrationTest , Unknown , }
    };
}

TestType!()