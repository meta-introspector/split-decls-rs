// Generated macro for impl_17 (impl)
macro_rules! Depcrateimpl_17 {
() => {
// Module: crate
// Provides: {"impl_17"}
// Dependencies: {}
impl App { const FRAMES_PER_SECOND : f32 = 60.0 ; pub async fn run (mut self , mut terminal : DefaultTerminal) -> Result < () > { self . pull_requests . run () ; let period = Duration :: from_secs_f32 (1.0 / Self :: FRAMES_PER_SECOND) ; let mut interval = tokio :: time :: interval (period) ; let mut events = EventStream :: new () ; while ! self . should_quit { tokio :: select ! { _ = interval . tick () => { terminal . draw (| frame | self . render (frame)) ?; } , Some (Ok (event)) = events . next () => self . handle_event (& event) , } } Ok (()) } fn render (& self , frame : & mut Frame) { let layout = Layout :: vertical ([Constraint :: Length (1) , Constraint :: Fill (1)]) ; let [title_area , body_area] = frame . area () . layout (& layout) ; let title = Line :: from ("Ratatui async example") . centered () . bold () ; frame . render_widget (title , title_area) ; frame . render_widget (& self . pull_requests , body_area) ; } fn handle_event (& mut self , event : & Event) { if let Some (key) = event . as_key_press_event () { match key . code { KeyCode :: Char ('q') | KeyCode :: Esc => self . should_quit = true , KeyCode :: Char ('j') | KeyCode :: Down => self . pull_requests . scroll_down () , KeyCode :: Char ('k') | KeyCode :: Up => self . pull_requests . scroll_up () , _ => { } } } } }
};
}
