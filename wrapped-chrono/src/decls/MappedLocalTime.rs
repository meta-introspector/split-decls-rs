macro_rules! deps {
    () => {
        Local!();
        DateTime!();
        LocalResult!();
    };
}

macro_rules! MappedLocalTime {
    () => {
        deps!();
        # [doc = " The result of mapping a local time to a concrete instant in a given time zone."] # [doc = ""] # [doc = " The calculation to go from a local time (wall clock time) to an instant in UTC can end up in"] # [doc = " three cases:"] # [doc = " * A single, simple result."] # [doc = " * An ambiguous result when the clock is turned backwards during a transition due to for example"] # [doc = "   DST."] # [doc = " * No result when the clock is turned forwards during a transition due to for example DST."] # [doc = ""] # [doc = " <div class=\"warning\">"] # [doc = ""] # [doc = " In wasm, when using [`Local`], only the [`LocalResult::Single`] variant is returned."] # [doc = " Specifically:"] # [doc = ""] # [doc = " * When the clock is turned backwards, where `Ambiguous(earliest, latest)` would be expected,"] # [doc = "   `Single(earliest)` is returned instead."] # [doc = " * When the clock is turned forwards, where `None` would be expected, `Single(t)` is returned,"] # [doc = "   with `t` being the requested local time represented as though there is no transition on that"] # [doc = "   day (i.e. still \"summer time\")"] # [doc = ""] # [doc = " This is caused because of limitations in the JavaScript"] # [doc = " [`Date`](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Date)"] # [doc = " API, which always parses a local time as a single, valid time - even for an"] # [doc = " input which describes a nonexistent or ambiguous time."] # [doc = ""] # [doc = " See further discussion and workarounds in <https://github.com/chronotope/chrono/issues/1701>."] # [doc = ""] # [doc = " </div>"] # [doc = ""] # [doc = " When the clock is turned backwards it creates a _fold_ in local time, during which the local"] # [doc = " time is _ambiguous_. When the clock is turned forwards it creates a _gap_ in local time, during"] # [doc = " which the local time is _missing_, or does not exist."] # [doc = ""] # [doc = " Chrono does not return a default choice or invalid data during time zone transitions, but has"] # [doc = " the `MappedLocalTime` type to help deal with the result correctly."] # [doc = ""] # [doc = " The type of `T` is usually a [`DateTime`] but may also be only an offset."] pub type MappedLocalTime < T > = LocalResult < T > ;
    };
}

MappedLocalTime!();