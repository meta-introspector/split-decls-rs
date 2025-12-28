macro_rules! Sink {
    () => {
        # [doc = " Writer for the [`sink()`] function."] # [must_use = "writers do nothing unless polled"] pub struct Sink { _priv : () , }
    };
}

Sink!()