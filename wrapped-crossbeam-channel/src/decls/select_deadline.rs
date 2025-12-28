macro_rules! deps {
    () => {
        SelectHandle!();
        SelectTimeoutError!();
        Timeout!();
        SelectedOperation!();
    };
}

macro_rules! select_deadline {
    () => {
        deps!();
        # [doc = " Blocks until a given deadline, or until one of the operations becomes ready and selects it."] # [inline] pub (crate) fn select_deadline < 'a > (handles : & mut [(& 'a dyn SelectHandle , usize , * const u8)] , deadline : Instant , is_biased : bool ,) -> Result < SelectedOperation < 'a > , SelectTimeoutError > { match run_select (handles , Timeout :: At (deadline) , is_biased) { None => Err (SelectTimeoutError) , Some ((token , index , ptr)) => Ok (SelectedOperation { token , index , ptr , _marker : PhantomData , }) , } }
    };
}

select_deadline!()