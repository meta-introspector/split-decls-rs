// Generated macro for REQUEST_TABLE_INIT_SCRIPT (const)
macro_rules! Depcrate_reports_htmlREQUEST_TABLE_INIT_SCRIPT {
() => {
// Module: crate::reports::html
// Provides: {"REQUEST_TABLE_INIT_SCRIPT"}
// Dependencies: {}
const REQUEST_TABLE_INIT_SCRIPT : & str = r#"
<script type="text/javascript">
    window.addEventListener("load", (event) => {

        let prefers = window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light';
        let html = document.querySelector('html');

        html.classList.add(prefers);
        html.setAttribute('data-bs-theme', prefers);

        new DataTable('table.log-dancer-table',
        {
            paging: false,
            dom: '<"center" flpti  >',
            columnDefs: [
                {targets: [0,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19], type: 'html-num'}
            ]
        });

        let loading = document.getElementById("loading");
        loading.style.visibility = 'hidden';

        let tables = document.getElementById("tables");
        tables.style.visibility = 'visible';
    });
</script>
"# ;
};
}
