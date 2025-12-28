macro_rules! deps {
    () => {
        BenchmarkId!();
        ReportContext!();
    };
}

macro_rules! PlotContext {
    () => {
        deps!();
        # [derive (Clone , Copy)] pub (crate) struct PlotContext < 'a > { pub (crate) id : & 'a BenchmarkId , pub (crate) context : & 'a ReportContext , pub (crate) size : Option < (usize , usize) > , pub (crate) is_thumbnail : bool , }
    };
}

PlotContext!()