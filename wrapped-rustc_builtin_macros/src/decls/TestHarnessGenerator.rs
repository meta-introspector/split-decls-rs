macro_rules! deps {
    () => {
        Test!();
        TestCtxt!();
    };
}

macro_rules! TestHarnessGenerator {
    () => {
        deps!();
        struct TestHarnessGenerator < 'a > { cx : TestCtxt < 'a > , tests : Vec < Test > , }
    };
}

TestHarnessGenerator!();