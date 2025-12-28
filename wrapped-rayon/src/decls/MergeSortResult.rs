macro_rules! MergeSortResult {
    () => {
        # [doc = " The result of merge sort."] # [must_use] # [derive (Clone , Copy , PartialEq , Eq)] enum MergeSortResult { # [doc = " The slice has already been sorted."] NonDescending , # [doc = " The slice has been descending and therefore it was left intact."] Descending , # [doc = " The slice was sorted."] Sorted , }
    };
}

MergeSortResult!()