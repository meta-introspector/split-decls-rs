macro_rules! Green {
    () => {
        enum Green { Node { ptr : Cell < ptr :: NonNull < GreenNodeData > > } , Token { ptr : ptr :: NonNull < GreenTokenData > } , }
    };
}

Green!()