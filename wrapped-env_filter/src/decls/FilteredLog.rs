macro_rules! deps {
    () => {
        Filter!();
    };
}

macro_rules! FilteredLog {
    () => {
        deps!();
        # [doc = " Decorate a [`log::Log`] with record [`Filter`]ing."] # [doc = ""] # [doc = " Records that match the filter will be forwarded to the wrapped log."] # [doc = " Other records will be ignored."] # [derive (Debug)] pub struct FilteredLog < T > { log : T , filter : Filter , }
    };
}

FilteredLog!();