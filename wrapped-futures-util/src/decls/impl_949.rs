macro_rules! impl_949 {
    () => {
        impl < S : Stream , F > Stream for SinkMapErr < S , F > { type Item = S :: Item ; delegate_stream ! (sink) ; }
    };
}

impl_949!()