use anyhow::Result;

mod simple_test;
mod function_caller;
mod test_extracted_functions_v2;

pub fn test_bootstrap3() -> Result<()> {
    simple_test::test_extracted_function()?;
    test_extracted_functions_v2::test_process_crates_function()?;
    function_caller::call_all_functions()
}
