macro_rules! deps {
    () => {
        Gen!();
        TestResult!();
    };
}

macro_rules! Testable {
    () => {
        deps!();
        # [doc = " `Testable` describes types (e.g., a function) whose values can be"] # [doc = " tested."] # [doc = ""] # [doc = " Anything that can be tested must be capable of producing a `TestResult`"] # [doc = " given a random number generator. This is trivial for types like `bool`,"] # [doc = " which are just converted to either a passing or failing test result."] # [doc = ""] # [doc = " For functions, an implementation must generate random arguments"] # [doc = " and potentially shrink those arguments if they produce a failure."] # [doc = ""] # [doc = " It's unlikely that you'll have to implement this trait yourself."] pub trait Testable : 'static { fn result (& self , _ : & mut Gen) -> TestResult ; }
    };
}

Testable!()