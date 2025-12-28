use anyhow::Result;

mod simple_test;

pub fn test_bootstrap3() -> Result<()> {
    simple_test::test_extracted_function()
}
