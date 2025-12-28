macro_rules! UpdateTest {
    () => {
        # [derive (Debug , Default , Clone , Copy , PartialEq , Eq , Hash)] pub struct UpdateTest { pub expect_test : bool , pub insta : bool , pub snapbox : bool , }
    };
}

UpdateTest!()