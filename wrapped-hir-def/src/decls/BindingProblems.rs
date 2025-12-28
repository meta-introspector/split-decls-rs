macro_rules! BindingProblems {
    () => {
        # [derive (Debug , Clone , Eq , PartialEq)] pub enum BindingProblems { # [doc = " <https://doc.rust-lang.org/stable/error_codes/E0416.html>"] BoundMoreThanOnce , # [doc = " <https://doc.rust-lang.org/stable/error_codes/E0409.html>"] BoundInconsistently , # [doc = " <https://doc.rust-lang.org/stable/error_codes/E0408.html>"] NotBoundAcrossAll , }
    };
}

BindingProblems!();