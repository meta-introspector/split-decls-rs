macro_rules! Local {
    () => {
        # [doc = " Thread local data for the helping strategy."] # [derive (Default)] pub (super) struct Local { generation : Cell < usize > , }
    };
}

Local!()