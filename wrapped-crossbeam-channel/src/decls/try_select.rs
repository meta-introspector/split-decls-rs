macro_rules! deps {
    () => {
        Timeout!();
        TrySelectError!();
        SelectHandle!();
        SelectedOperation!();
    };
}

macro_rules! try_select {
    () => {
        deps!();
        # [doc = " Attempts to select one of the operations without blocking."] # [inline] pub fn try_select < 'a > (handles : & mut [(& 'a dyn SelectHandle , usize , * const u8)] , is_biased : bool ,) -> Result < SelectedOperation < 'a > , TrySelectError > { match run_select (handles , Timeout :: Now , is_biased) { None => Err (TrySelectError) , Some ((token , index , ptr)) => Ok (SelectedOperation { token , index , ptr , _marker : PhantomData , }) , } }
    };
}

try_select!()