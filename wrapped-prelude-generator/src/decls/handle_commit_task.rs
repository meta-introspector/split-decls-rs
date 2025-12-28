macro_rules! deps {
    () => {
        Args!();
    };
}

macro_rules! handle_commit_task {
    () => {
        deps!();
        pub async fn handle_commit_task (_args : & Args , task_id : & str) -> Result < () > { println ! ("Committing task: {}" , task_id) ; Ok (()) }
    };
}

handle_commit_task!()