// Generated macro for default_mode_is_unchanged (function)
macro_rules! Depcrate_heading_testsdefault_mode_is_unchanged {
() => {
// Module: crate::heading::tests
// Provides: {"default_mode_is_unchanged"}
// Dependencies: {}
# [test] fn default_mode_is_unchanged () { let result = rewrite_headings ("# This is *emphasized* and **strong** and `code`
## Here is *another* and **strong** and `code`
### Third *level* **heading** with `code`
#### Fourth *heading* **level** and `code`
##### Fifth *level* **heading** and `code`
###### Last *heading* **level** with `code`
" , Mode :: Default ,) ; assert_eq ! (result . unwrap () , "# This is *emphasized* and **strong** and `code`
## Here is *another* and **strong** and `code`
### Third *level* **heading** with `code`
#### Fourth *heading* **level** and `code`
##### Fifth *level* **heading** and `code`
###### Last *heading* **level** with `code`
") ; }
};
}
