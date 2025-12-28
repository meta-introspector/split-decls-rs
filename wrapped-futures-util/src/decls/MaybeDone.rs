macro_rules! MaybeDone {
    () => {
        # [doc = " A future that may have completed."] # [doc = ""] # [doc = " This is created by the [`maybe_done()`] function."] # [derive (Debug)] pub enum MaybeDone < Fut : Future > { # [doc = " A not-yet-completed future"] Future (Fut) , # [doc = " The output of the completed future"] Done (Fut :: Output) , # [doc = " The empty variant after the result of a [`MaybeDone`] has been"] # [doc = " taken using the [`take_output`](MaybeDone::take_output) method."] Gone , }
    };
}

MaybeDone!()