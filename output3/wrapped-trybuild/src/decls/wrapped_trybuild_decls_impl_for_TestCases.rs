use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl TestCases {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        TestCases {
            runner: RefCell::new(Runner { tests: Vec::new() }),
        }
    }
    pub fn pass<P: AsRef<Path>>(&self, path: P) {
        self.runner.borrow_mut().tests.push(Test {
            path: path.as_ref().to_owned(),
            expected: Expected::Pass,
        });
    }
    pub fn compile_fail<P: AsRef<Path>>(&self, path: P) {
        self.runner.borrow_mut().tests.push(Test {
            path: path.as_ref().to_owned(),
            expected: Expected::CompileFail,
        });
    }
}
