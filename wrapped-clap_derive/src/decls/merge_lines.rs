macro_rules! deps {
    () => {
        Item!();
    };
}

macro_rules! merge_lines {
    () => {
        deps!();
        # [cfg (not (feature = "unstable-markdown"))] fn merge_lines (lines : impl IntoIterator < Item = impl AsRef < str > >) -> String { lines . into_iter () . map (| s | s . as_ref () . trim () . to_owned ()) . collect :: < Vec < _ > > () . join (" ") }
    };
}

merge_lines!();