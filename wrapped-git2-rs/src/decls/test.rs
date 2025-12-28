macro_rules! test {
    () => {
        # [cfg (test)] # [macro_use] mod test ;
    };
}

test!()