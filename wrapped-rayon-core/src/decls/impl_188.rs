macro_rules! deps {
    () => {
        ScopeBase!();
        HeapJob!();
        WorkerThread!();
        JobFifo!();
        BroadcastContext!();
        ScopePtr!();
        ScopeFifo!();
        Registry!();
        ArcJob!();
    };
}

macro_rules! impl_188 {
    () => {
        deps!();
        impl < 'scope > ScopeFifo < 'scope > { fn new (owner : Option < & WorkerThread > , registry : Option < & Arc < Registry > >) -> Self { let base = ScopeBase :: new (owner , registry) ; let num_threads = base . registry . num_threads () ; let fifos = (0 .. num_threads) . map (| _ | JobFifo :: new ()) . collect () ; ScopeFifo { base , fifos } } # [doc = " Spawns a job into the fork-join scope `self`. This job will"] # [doc = " execute sometime before the fork-join scope completes.  The"] # [doc = " job is specified as a closure, and this closure receives its"] # [doc = " own reference to the scope `self` as argument. This can be"] # [doc = " used to inject new jobs into `self`."] # [doc = ""] # [doc = " # See also"] # [doc = ""] # [doc = " This method is akin to [`Scope::spawn()`], but with a FIFO"] # [doc = " priority.  The [`scope_fifo` function] has more details about"] # [doc = " this distinction."] # [doc = ""] # [doc = " [`scope_fifo` function]: scope_fifo()"] pub fn spawn_fifo < BODY > (& self , body : BODY) where BODY : FnOnce (& ScopeFifo < 'scope >) + Send + 'scope , { let scope_ptr = ScopePtr (self) ; let job = HeapJob :: new (move | | unsafe { let scope = scope_ptr . as_ref () ; ScopeBase :: execute_job (& scope . base , move | | body (scope)) }) ; let job_ref = self . base . heap_job_ref (job) ; match self . base . registry . current_thread () { Some (worker) => { let fifo = & self . fifos [worker . index ()] ; unsafe { worker . push (fifo . push (job_ref)) } ; } None => self . base . registry . inject (job_ref) , } } # [doc = " Spawns a job into every thread of the fork-join scope `self`. This job will"] # [doc = " execute on each thread sometime before the fork-join scope completes.  The"] # [doc = " job is specified as a closure, and this closure receives its own reference"] # [doc = " to the scope `self` as argument, as well as a `BroadcastContext`."] pub fn spawn_broadcast < BODY > (& self , body : BODY) where BODY : Fn (& ScopeFifo < 'scope > , BroadcastContext < '_ >) + Send + Sync + 'scope , { let scope_ptr = ScopePtr (self) ; let job = ArcJob :: new (move | | unsafe { let scope = scope_ptr . as_ref () ; let body = & body ; let func = move | | BroadcastContext :: with (move | ctx | body (scope , ctx)) ; ScopeBase :: execute_job (& scope . base , func) }) ; self . base . inject_broadcast (job) } }
    };
}

impl_188!()